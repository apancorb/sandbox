export const getError = (type: string, e: Error): any => {
  return {
    type: type,
    name: e.name,
    message: e.message,
    stack: e.stack,
  };
};
