import { ImageAsset } from "@nativescript/canvas";
import { SVGItem } from "./SVGItem";

const b64Extensions = {
	"/": "jpg",
	i: "png",
	R: "gif",
	U: "webp",
};

function b64WithoutPrefix(b64) {
	return b64.split(",")[1];
}

function getMIMEforBase64String(b64) {
	let input = b64;
	if (b64.includes(",")) {
		input = b64WithoutPrefix(b64);
	}
	const first = input.charAt(0);
	const mime = b64Extensions[first];
	if (!mime) {
		throw new Error("Unknown Base64 MIME type: " + b64);
	}
	return mime;
}

export class Image extends SVGItem {
	xlink: { href?: string } = {};
	href: string;
	x: any = 0;
	y: any = 0;
	#loadedSrc: string;
	#asset = new ImageAsset();
}
