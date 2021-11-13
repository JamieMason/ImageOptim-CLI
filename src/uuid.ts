/**
 * Cheaply generate a UUID with a low chance of collisions
 * https://stackoverflow.com/a/8809472/745158
 */
export function uuid(): string {
  let epoch: number = new Date().getTime();
  let sessionLength: number = Date.now() * 1000 || 0;
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (character) => {
    let randomNumber: number = Math.random() * 16;
    if (epoch > 0) {
      randomNumber = (epoch + randomNumber) % 16 | 0;
      epoch = Math.floor(epoch / 16);
    } else {
      randomNumber = (sessionLength + randomNumber) % 16 | 0;
      sessionLength = Math.floor(sessionLength / 16);
    }
    return (character == 'x' ? randomNumber : (randomNumber & 0x7) | 0x8).toString(16);
  });
}
