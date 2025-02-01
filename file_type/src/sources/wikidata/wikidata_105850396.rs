use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850396: FileFormat = FileFormat {
    id: 105_850_396,
    puid: "wikidata/105850396",
    name: "Programmer's Notepad text Clips",
    extensions: &["clips"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
