use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47519823: FileFormat = FileFormat {
    id: 47_519_823,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version 6",
    extensions: &["ppp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
