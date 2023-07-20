use futures::executor::block_on;
//use futures::executor::block_on;
use parking_lot::Mutex;
use super::actor_workers::*;
use std::{sync::Arc, ops::DerefMut};
use tokio::task::{self, JoinHandle, spawn_blocking};

//do_work/returning_do_work

pub struct Actor<T>
{

    data: Arc<Mutex<T>>   

}

impl<T> Actor<T> where
    T: std::marker::Send + 'static
{

    pub fn new(object: T) -> Self
    {

        Actor
        {

            data: Arc::new(Mutex::new(object))

        }

    }

    pub fn do_work<W, R>(&self, mut worker: W) -> JoinHandle<(R, W)> where //mut 
        W: ReturningActorWorker<T, R> + std::marker::Send + 'static,
        R: std::marker::Send + 'static
    {

        let data_cl = self.data.clone();

        tokio::spawn(async move {

            let res;

            {
                let mut mtg = data_cl.lock(); //.unwrap();;

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
                let mut mtg = data_cl.lock(); //.unwrap();

                let data = mtg.deref_mut();
            
                res = <W as ReturningActorWorker<T, R>>::work(&mut worker, data);

            }

            (res, worker)

        })

    }

    pub fn async_do_work<W, R>(&mut self, mut worker: W) -> JoinHandle<(R, W)> where
        W:  ReturningAsyncActorWorker<T, R> + std::marker::Send + 'static,
        R: std::marker::Send + 'static
    {

        let data_cl = self.data.clone();

        tokio::spawn(async move {

            let res;

            {
                let mut mtg = data_cl.lock();

                let data = mtg.deref_mut();
            
                let res_f = <W as  ReturningAsyncActorWorker<T, R>>::work(&mut worker, data);

                //res = res_f.await;

                res = block_on(res_f);
                
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
                let mut mtg = data_cl.lock();

                let data = mtg.deref_mut();
            
                res = block_on(<W as ReturningAsyncActorWorker<T, R>>::work(&mut worker, data));

            }

            (res, worker)

        })

    }

}