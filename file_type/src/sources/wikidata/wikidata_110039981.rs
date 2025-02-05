use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110039981: FileFormat = FileFormat {
    id: 110_039_981,
    source_type: SourceType::Wikidata,
    name: "Phantom CINE Compressed Video File",
    extensions: &["cci"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
