use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600717: FileFormat = FileFormat {
    id: 28_600_717,
    source_type: SourceType::Wikidata,
    name: "DrawIt",
    extensions: &["drawit"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
