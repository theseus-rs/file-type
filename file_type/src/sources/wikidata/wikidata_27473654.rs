use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473654: FileFormat = FileFormat {
    id: 27_473_654,
    source_type: SourceType::Wikidata,
    name: "Band Interleaved by Pixel Image File",
    extensions: &["bip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
