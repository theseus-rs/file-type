use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113495271: FileFormat = FileFormat {
    id: 113_495_271,
    source_type: SourceType::Wikidata,
    name: "Applet Effect Factory Config File",
    extensions: &["data"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
