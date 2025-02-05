use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_43870624: FileFormat = FileFormat {
    id: 43_870_624,
    source_type: SourceType::Wikidata,
    name: "PCX, version 5",
    extensions: &["pcc", "pcx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
