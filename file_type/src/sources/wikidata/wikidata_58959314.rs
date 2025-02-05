use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58959314: FileFormat = FileFormat {
    id: 58_959_314,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Theme",
    extensions: &["thmx"],
    media_types: &["application/vnd.ms-officetheme"],
    signatures: &[],
    related_formats: &[],
};
