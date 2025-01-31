use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111418333: FileFormat = FileFormat {
    id: 111_418_333,
    puid: "wikidata/111418333",
    name: "Adobe Bridge URL File",
    extensions: &["adobebridge"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
