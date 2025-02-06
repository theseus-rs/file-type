use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21040751: FileFormat = FileFormat {
    id: 21_040_751,
    source_type: SourceType::Wikidata,
    name: "Farandole Composer format",
    extensions: &["far"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
