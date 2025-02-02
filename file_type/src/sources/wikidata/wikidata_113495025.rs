use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113495025: FileFormat = FileFormat {
    id: 113_495_025,
    source_type: SourceType::Wikidata,
    name: "Software602 Printer Configuration File",
    extensions: &["cfg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
