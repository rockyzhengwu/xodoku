import axios from "axios";

export async function getImageAsBase64(imageUrl) {
  try {
    const response = await axios.get(imageUrl, {
      responseType: "arraybuffer",
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Request-Method": "GET",
        "Access-Control-Allow-Credentials": true,
      },
    });

    const buffer = Buffer.from(response.data);

    const base64Image = buffer.toString("base64");
    return base64Image;
  } catch (error) {
    console.error("Error fetching image:", error);
    return null;
  }
}
