use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_74673954: FileFormat = FileFormat {
    id: 74_673_954,
    puid: "wikidata/74673954",
    name: "TurboTax return file",
    extensions: &["tax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
