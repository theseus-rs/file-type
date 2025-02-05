use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206548: FileFormat = FileFormat {
    id: 28_206_548,
    source_type: SourceType::Wikidata,
    name: "MAKIchan Graphics MAX",
    extensions: &["max"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
