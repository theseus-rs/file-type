use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122676986: FileFormat = FileFormat {
    id: 122_676_986,
    source_type: SourceType::Wikidata,
    name: "CMX Corel Clipart",
    extensions: &["cmx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
