use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131341382: FileFormat = FileFormat {
    id: 131_341_382,
    puid: "wikidata/131341382",
    name: "Typst code",
    extensions: &["typ"],
    media_types: &["text/x-typst"],
    internal_signatures: &[],
    related_formats: &[],
};
