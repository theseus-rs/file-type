use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105047785: FileFormat = FileFormat {
    id: 105_047_785,
    puid: "wikidata/105047785",
    name: "Binary Color Format",
    extensions: &["bcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
