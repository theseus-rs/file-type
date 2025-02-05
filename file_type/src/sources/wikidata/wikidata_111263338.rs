use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111263338: FileFormat = FileFormat {
    id: 111_263_338,
    source_type: SourceType::Wikidata,
    name: "DirectMusic Producer DLS file",
    extensions: &["dlp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
