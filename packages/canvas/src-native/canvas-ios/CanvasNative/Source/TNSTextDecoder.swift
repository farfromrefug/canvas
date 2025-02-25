//
//  TNSTextDecoder.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/14/20.
//

import Foundation
@objcMembers
@objc(TNSTextDecoder)
public class TNSTextDecoder: NSObject {

    private var decoder: Int64 = 0
    public override init() {
        super.init()
        create(encoding: "utf-8")
    }

    public init(encoding: String){
        super.init()
        create(encoding: encoding)
    }

    private func create(encoding: String){
        let type = (encoding as NSString).utf8String
        decoder = text_decoder_create(type)
    }

    public var encoding: String {
        let raw = text_decoder_get_encoding(decoder)
        if(raw == nil){
            // Return default utf8 ?
            return String()
        }
        let encoding = String(cString: raw!)
        destroy_string(raw)
        return encoding
    }

    public func decode(buffer: Data) -> String{
        var data = [UInt8](buffer)
        let raw = text_decoder_decode(decoder, &data, UInt(buffer.count))
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        destroy_string(raw)
        return result
    }

    public func decode(bytes: [UInt8]) -> String{
        var data = bytes
        let raw = text_decoder_decode(decoder, &data, UInt(bytes.count))
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        destroy_string(raw)
        return result
    }


    public func decode(i8 bytes: [Int8]) -> String{
        let data = bytes.withUnsafeBytes { (buf) -> UnsafePointer<UInt8>? in
            return buf.baseAddress?.assumingMemoryBound(to: UInt8.self)
        }
        let raw = text_decoder_decode(decoder, data, UInt(bytes.count))
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        destroy_string(raw)
        return result
    }



    public func decode(u16 bytes: [UInt16]) -> String {
        var data = bytes
        let raw = text_decoder_decode_u16(decoder, &data, UInt(bytes.count))
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        destroy_string(raw)
        return result
    }


    public func decode(i16 bytes: [Int16]) -> String{
        var data = bytes
        let raw = text_decoder_decode_i16(decoder, &data, UInt(bytes.count))
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        destroy_string(raw)
        return result
    }



    public func decode(i32 bytes: [Int32]) -> String{
        let data = bytes
        let raw = text_decoder_decode_i32(decoder, data, UInt(bytes.count))
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        destroy_string(raw)
        return result
    }

    deinit {
        destroy_text_decoder(decoder)
    }

}
