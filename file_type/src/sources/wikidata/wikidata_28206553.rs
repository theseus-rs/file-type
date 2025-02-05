use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206553: FileFormat = FileFormat {
    id: 28_206_553,
    source_type: SourceType::Wikidata,
    name: "MAKIchan Graphics MKI",
    extensions: &["mag", "max", "mki"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
