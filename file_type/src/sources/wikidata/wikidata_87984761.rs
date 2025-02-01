use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87984761: FileFormat = FileFormat {
    id: 87_984_761,
    puid: "wikidata/87984761",
    name: "CorelCHART Document 3-4",
    extensions: &["cch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
