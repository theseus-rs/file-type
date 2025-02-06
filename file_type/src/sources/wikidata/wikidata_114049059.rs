use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114049059: FileFormat = FileFormat {
    id: 114_049_059,
    source_type: SourceType::Wikidata,
    name: "Rocky Interlace Picture",
    extensions: &["rip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
