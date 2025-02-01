use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73514919: FileFormat = FileFormat {
    id: 73_514_919,
    puid: "wikidata/73514919",
    name: "Package Resource Index",
    extensions: &["pri"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
