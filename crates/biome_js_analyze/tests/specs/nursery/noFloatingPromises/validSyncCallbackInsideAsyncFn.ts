/* should not generate diagnostics */

async function doStuff(db) {
    const txStatements: Array<(tx: any) => void> = [(tx) => tx.insert().run()];

    db.transaction((tx: any) => {
        for (const stmt of txStatements) {
            stmt(tx)
        }
    });
}

async function doStuff2(db) {
    const txStatements: Array<Promise<(tx: any) => void>> = [];

    db.transaction(async (tx: any) => {
        for await (const stmt of txStatements) {
            stmt(tx)
        }
    });
}
