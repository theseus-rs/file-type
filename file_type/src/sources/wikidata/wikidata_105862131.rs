use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862131: FileFormat = FileFormat {
    id: 105_862_131,
    source_type: SourceType::Wikidata,
    name: "Emacs Muse project",
    extensions: &["muse"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
