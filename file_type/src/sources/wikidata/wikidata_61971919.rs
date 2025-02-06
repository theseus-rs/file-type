use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61971919: FileFormat = FileFormat {
    id: 61_971_919,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual FoxPro Database Table File",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
