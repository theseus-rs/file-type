use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58335745: FileFormat = FileFormat {
    id: 58_335_745,
    source_type: SourceType::Wikidata,
    name: "Acrobat Catalog Cat File",
    extensions: &["cat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
