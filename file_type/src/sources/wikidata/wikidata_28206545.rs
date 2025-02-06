use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206545: FileFormat = FileFormat {
    id: 28_206_545,
    source_type: SourceType::Wikidata,
    name: "MAKIchan Graphics MAG",
    extensions: &["mag"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
