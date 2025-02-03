use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51954521: FileFormat = FileFormat {
    id: 51_954_521,
    source_type: SourceType::Wikidata,
    name: "Microsoft FoxPro Database, version 2.6",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
