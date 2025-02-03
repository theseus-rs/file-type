use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110443175: FileFormat = FileFormat {
    id: 110_443_175,
    source_type: SourceType::Wikidata,
    name: "Visual Basics MAK File",
    extensions: &["mak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
