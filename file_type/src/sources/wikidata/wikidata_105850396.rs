use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850396: FileFormat = FileFormat {
    id: 105_850_396,
    source_type: SourceType::Wikidata,
    name: "Programmer's Notepad text Clips",
    extensions: &["clips"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
