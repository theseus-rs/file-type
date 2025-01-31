use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110238400: FileFormat = FileFormat {
    id: 110_238_400,
    puid: "wikidata/110238400",
    name: "Screenwriter 6 file format",
    extensions: &["mmsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
