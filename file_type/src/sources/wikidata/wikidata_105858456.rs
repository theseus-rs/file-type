use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858456: FileFormat = FileFormat {
    id: 105_858_456,
    puid: "wikidata/105858456",
    name: "Acrobat eBook Reader EBX Transfer Data",
    extensions: &["etd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
