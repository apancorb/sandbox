export const sleep = (
    minutes: number
): Promise<void> => {
    return new Promise(
        resolve => setTimeout(
            resolve,
            minutes * 60000
        )
    );
};

export const priceDiff = (
    price0: number | string,
    price1: number | string,
    abs: boolean = false
): number => {
    const p0: number = typeof price0 === "string" ?
        Number.parseFloat(price0) : price0;
    const p1: number = typeof price1 === "string" ?
        Number.parseFloat(price1) : price1;
    const res: number = ((p1 - p0) / p0) * 100;
    return abs ? Math.abs(res) : res;
};

export const msToHMS = (
    ms: number
): string => {
    // convert to seconds:
    let seconds: number | string = ms / 1000;
    // extract hours:
    const hours: string = (seconds / 3600).toFixed(2); // 3,600 seconds in 1 hour
    seconds = seconds % 3600; // seconds remaining after extracting hours
    // extract minutes:
    const minutes: string = (seconds / 60).toFixed(2); // 60 seconds in 1 minute
    // keep only seconds not extracted to minutes:
    seconds = (seconds % 60).toFixed(2);
    // append all times
    return hours+":"+minutes+":"+seconds;
};
