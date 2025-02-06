use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_97033396: FileFormat = FileFormat {
    id: 97_033_396,
    source_type: SourceType::Wikidata,
    name: "Magick Vector Graphics",
    extensions: &["mvg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
