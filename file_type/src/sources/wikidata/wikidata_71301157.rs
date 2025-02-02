use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71301157: FileFormat = FileFormat {
    id: 71_301_157,
    source_type: SourceType::Wikidata,
    name: "WHIP! DWF Format",
    extensions: &["dwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
