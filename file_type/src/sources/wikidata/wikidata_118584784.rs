use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118584784: FileFormat = FileFormat {
    id: 118_584_784,
    source_type: SourceType::Wikidata,
    name: "Cakewalk Bundle",
    extensions: &["cwb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
