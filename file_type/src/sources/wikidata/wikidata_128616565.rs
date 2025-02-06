use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128616565: FileFormat = FileFormat {
    id: 128_616_565,
    source_type: SourceType::Wikidata,
    name: "Asymptote file format",
    extensions: &["asy"],
    media_types: &["text/x-asymptote"],
    signatures: &[],
    related_formats: &[],
};
