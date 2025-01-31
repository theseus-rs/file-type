use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206553: FileFormat = FileFormat {
    id: 28_206_553,
    puid: "wikidata/28206553",
    name: "MAKIchan Graphics MKI",
    extensions: &["mag", "max", "mki"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
