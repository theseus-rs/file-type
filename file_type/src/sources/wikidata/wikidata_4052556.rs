use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4052556: FileFormat = FileFormat {
    id: 4_052_556,
    puid: "wikidata/4052556",
    name: "Vector Quantized Animation",
    extensions: &["vqa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
