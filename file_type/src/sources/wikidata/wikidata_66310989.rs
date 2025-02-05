use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66310989: FileFormat = FileFormat {
    id: 66_310_989,
    source_type: SourceType::Wikidata,
    name: "Pantry Files",
    extensions: &["pl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
