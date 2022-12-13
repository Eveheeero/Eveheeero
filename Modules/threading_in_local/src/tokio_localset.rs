pub(super) fn localset() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let data = "123";
    let local = tokio::task::LocalSet::new();

    // localset block_on with runtime
    local.block_on(&runtime, async move {
        println!("localset 1: {}", data);
    });

    // localset block_on with runtime
    runtime.block_on(local.run_until(async move {
        tokio::task::spawn_local(async move {
            println!("localset 2: {}", data);
        })
        .await
        .unwrap()
    }));

    // spawn_local 이후 block_on으로 await
    local.spawn_local(async move {
        println!("localset 3: {}", data);
    });
    runtime.block_on(async move { local.await });

    // 해당 방법이 실패한 이유는 spawn_local을 소환했는데, 이는 다른 localset 내부에서 발동되는것이기 때문?
    // local.spawn_local(async move {
    //     println!("Not Work: {}", data);
    // });
    // runtime.block_on(async move { local.enter() });

    // 이것이 실패한 이유는 모르겠다.
    // let handle = local.spawn_local(async move {
    //     println!("Not Work: {}", data);
    // });
    // runtime.block_on(async move { handle.await.unwrap() });

    drop(data);
}
