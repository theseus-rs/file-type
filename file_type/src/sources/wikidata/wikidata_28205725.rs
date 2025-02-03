use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205725: FileFormat = FileFormat {
    id: 28_205_725,
    source_type: SourceType::Wikidata,
    name: "Async Professional Fax",
    extensions: &["apf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
