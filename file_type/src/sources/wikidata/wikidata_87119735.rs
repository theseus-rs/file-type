use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87119735: FileFormat = FileFormat {
    id: 87_119_735,
    source_type: SourceType::Wikidata,
    name: "Open Financial Exchange 1.03",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx"],
    signatures: &[],
    related_formats: &[],
};
