use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850396: FileFormat = FileFormat {
    id: 105_850_396,
    source_type: SourceType::Wikidata,
    name: "Programmer's Notepad text Clips",
    extensions: &["clips"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
