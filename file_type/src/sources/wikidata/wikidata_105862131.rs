use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862131: FileFormat = FileFormat {
    id: 105_862_131,
    source_type: SourceType::Wikidata,
    name: "Emacs Muse project",
    extensions: &["muse"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
