use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47895228: FileFormat = FileFormat {
    id: 47_895_228,
    source_type: SourceType::Wikidata,
    name: "Visual Basic Macro",
    extensions: &["dvb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
