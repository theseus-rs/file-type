use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_107649162: FileFormat = FileFormat {
    id: 107_649_162,
    source_type: SourceType::Wikidata,
    name: "mzMLb",
    extensions: &["mzMLb"],
    media_types: &["application/x-hdf5"],
    signatures: &[],
    related_formats: &[],
};
