use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28551228: FileFormat = FileFormat {
    id: 28_551_228,
    source_type: SourceType::Wikidata,
    name: "Adobe Action File",
    extensions: &["atn"],
    media_types: &["application/x-photoshop"],
    internal_signatures: &[],
    related_formats: &[],
};
