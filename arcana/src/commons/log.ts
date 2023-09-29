import { Logger, configure, getLogger as _getLogger } from "log4js";

export const getLogger = (path: string, date: Date = new Date()): Logger => {
  configure({
    appenders: {
      log: {
        type: "file",
        filename: getPath(path, date, "log"),
      },
    },
    categories: {
      default: {
        appenders: ["log"],
        level: "all",
      },
      info: {
        appenders: ["log"],
        level: "all",
      },
    },
  });
  return _getLogger("log");
};

const getPath = (prefix: string, date: Date, posfix: string): string => {
  return (
    `${prefix}/` +
    `${date.getUTCFullYear()}/` +
    `${date.getUTCMonth()}/` +
    `${date.getUTCDate()}/` +
    `${posfix}`
  );
};
