use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122150098: FileFormat = FileFormat {
    id: 122_150_098,
    puid: "wikidata/122150098",
    name: "TaxAct 2008 Tax Return Backup File",
    extensions: &["ba8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
