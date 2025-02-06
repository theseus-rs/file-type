use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48906245: FileFormat = FileFormat {
    id: 48_906_245,
    source_type: SourceType::Wikidata,
    name: "Harvard Graphics Vector Graphics",
    extensions: &["cht"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
