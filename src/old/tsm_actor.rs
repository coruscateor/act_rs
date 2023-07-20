use futures::executor::block_on;
use tokio::sync::Mutex;
use crate::actor_workers::{ReturningActorWorker, ReturningAsyncActorWorker, AsyncActorWorker};

//use super::super::actor_workers::*; //ActorWorker;
use std::{sync::Arc, ops::DerefMut};
use tokio::task::{self, JoinHandle, spawn_blocking};

//A tokio::sync::Mutex oriented Actor

pub struct TSMActor<T>
{

    data: Arc<Mutex<T>>   

}

impl<T> TSMActor<T> where
    T: std::marker::Send + 'static
{

    pub fn new(object: T) -> Self
    {

        TSMActor
        {

            data: Arc::new(Mutex::new(object))

        }

    }

    //sync

    pub fn do_work<W, R>(&self, mut worker: W) -> JoinHandle<(R, W)> where
        W: ReturningActorWorker<T, R> + std::marker::Send + 'static,
        R: std::marker::Send + 'static
    {

        let data_cl = self.data.clone();

        tokio::spawn(async move {

            let res;

            {
                let mut mtg = data_cl.lock().await;

                let data = mtg.deref_mut();
            
                res = <W as ReturningActorWorker<T, R>>::work(&mut worker, data);

            }

            (res, worker)

        })

    }

    pub fn do_work_blocking<W, R>(&mut self, mut worker: W) -> JoinHandle<(R, W)> where
        W: ReturningActorWorker<T, R> + std::marker::Send + 'static,
        R: std::marker::Send + 'static
    {

        let data_cl = self.data.clone();

        tokio::task::spawn_blocking(move || {

            let res;

            {
                let mut mtg = block_on(data_cl.lock());

                let data = mtg.deref_mut();
            
                res = <W as ReturningActorWorker<T, R>>::work(&mut worker, data);

            }

            (res, worker)

        })

    }

    //async

    pub fn async_do_work<W, R>(&mut self, mut worker: W) -> JoinHandle<(R, W)> where
        W: ReturningAsyncActorWorker<T, R> + std::marker::Send + 'static,
        R: std::marker::Send + 'static
    {

        let data_cl = self.data.clone();

        tokio::spawn(async move {

            let res;

            {
                let mut mtg = data_cl.lock().await;

                let data = mtg.deref_mut();
            
                res = <W as ReturningAsyncActorWorker<T, R>>::work(&mut worker, data).await;

            }

            (res, worker)

        })

    }

    pub fn async_do_work_blocking<W, R>(&mut self, mut worker: W) -> JoinHandle<(R, W)> where
        W: ReturningAsyncActorWorker<T, R> + std::marker::Send + 'static,
        R: std::marker::Send + 'static
    {

        let data_cl = self.data.clone();

        tokio::task::spawn_blocking(move || {

            let res;

            {
                let mut mtg = block_on(data_cl.lock());

                let data = mtg.deref_mut();
            
                res = block_on(<W as ReturningAsyncActorWorker<T, R>>::work(&mut worker, data));

            }

            (res, worker)

        })

    }

}


