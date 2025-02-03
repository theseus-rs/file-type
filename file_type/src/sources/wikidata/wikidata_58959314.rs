use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58959314: FileFormat = FileFormat {
    id: 58_959_314,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Theme",
    extensions: &["thmx"],
    media_types: &["application/vnd.ms-officetheme"],
    internal_signatures: &[],
    related_formats: &[],
};
