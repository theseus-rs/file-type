use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113495025: FileFormat = FileFormat {
    id: 113_495_025,
    source_type: SourceType::Wikidata,
    name: "Software602 Printer Configuration File",
    extensions: &["cfg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
